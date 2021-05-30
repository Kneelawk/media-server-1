import { Component, HostListener, OnInit } from '@angular/core';
import { BackendService } from "../backend.service";
import { DirectoryChild, EntryInfo } from "../backend.types";
import { ActivatedRoute, Router } from "@angular/router";
import { Location } from "@angular/common";
import { BROWSE_PATH } from "../paths";
import { Title } from "@angular/platform-browser";

@Component({
  selector: 'app-browse',
  templateUrl: './browse.component.html',
  styleUrls: ['./browse.component.scss']
})
export class BrowseComponent implements OnInit {

  name: string = 'Loading...'
  path: string = 'Loading...'
  state: string = 'none';
  hasParent: boolean = false;
  parentUrl: string = '';

  // directory attributes
  children: Array<DirectoryChild> = [];

  // error attributes
  error: string = '';

  // file attributes
  isMediaFile: boolean = false;
  fileUrl: string = '';

  constructor(private backend: BackendService, private route: ActivatedRoute, public router: Router, private title: Title) { }

  @HostListener('document:click', ['$event'])
  onDocumentClick(event: Event) {
    const target = event.target;
    if (target instanceof HTMLAnchorElement) {
      if (target.classList.contains('browse-link')) {
        event.preventDefault();
      }
    }
  }

  ngOnInit(): void {
    this.route.url.subscribe(_ => {
      this.loadPath(this.getPath());
    })
    this.loadPath(this.getPath());
  }

  navigateBack() {
    this.router.navigateByUrl(this.parentUrl).then(_ => {});
  }

  navigateTo(path: string) {
    const fullPath = `/tree${ path }`;
    this.router.navigateByUrl(fullPath).then(_ => {});
  }

  url(url: string): string {
    return BackendService.url(url)
  }

  private resetContent() {
    // this.name = 'Loading...';
    // this.path = 'Loading...';
    // this.children = [];
  }

  private loadPath(path: string) {
    this.resetContent();

    path = path.replace('%2B', '+');

    console.log(`Loading path: ${ path }`)

    this.backend.getIndexFile(path).subscribe(result => {
      const value = result.Ok;
      if (value != null) {
        this.handleEntry(value);
      }
    }, error => {
      let value: EntryInfo | null = error.error.Ok;
      if (value != null) {
        this.handleEntry(value);
      }
    });
  }

  private handleEntry(value: EntryInfo) {
    if (value.name == '') {
      this.name = 'Browse';
      this.hasParent = false;
      this.parentUrl = '';
    } else {
      this.name = value.name;
      this.title.setTitle(value.name);
      this.hasParent = true;

      let url = this.route.snapshot.url;
      console.log(`Current Url: [${url}]`)
      if (url[url.length - 1].path == '') {
        url = url.slice(0, url.length - 2);
      } else {
        url = url.slice(0, url.length - 1);
      }
      if (url.length > 0) {
        this.parentUrl = `/${BROWSE_PATH}/${url.join('/')}/`;
      } else {
        this.parentUrl = `/${BROWSE_PATH}/`;
      }
    }

    this.path = value.path_pretty;

    const dir = value.detail.Directory;
    if (dir != null) {
      this.children = dir.children;
      this.state = 'dir';
    } else {
      this.children = [];
    }

    const file = value.detail.File;
    if (file != null) {
      this.state = 'file';
      this.isMediaFile = file.mime_type.startsWith('video/');
      this.fileUrl = file.url;
    } else {
      this.isMediaFile = false;
      this.fileUrl = '';
    }

    const error = value.detail.Error;
    if (error != null) {
      this.state = 'error';
      this.error = error.error;
    }
  }

  private getPath() {
    const snapshot = this.route.snapshot;

    if (snapshot.routeConfig?.path != '**') {
      return '/';
    } else {
      return '/' + snapshot.url.join('/')
    }
  }
}
