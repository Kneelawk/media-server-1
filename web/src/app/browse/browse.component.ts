import { Component, HostListener, OnInit } from '@angular/core';
import { BackendService } from "../backend.service";
import { EntryDetail, EntryInfo, ResponseResult } from "../backend.types";
import { ActivatedRoute, Router } from "@angular/router";
import { BROWSE_PATH } from "../paths";
import { Title } from "@angular/platform-browser";
import { environment } from "../../environments/environment";

@Component({
  selector: 'app-browse',
  templateUrl: './browse.component.html',
  styleUrls: ['./browse.component.scss']
})
export class BrowseComponent implements OnInit {

  name: string = 'Loading...'
  path: string = 'Loading...'
  state: 'none' | 'directory' | 'error' | 'file' | 'media-file' = 'none';
  hasParent: boolean = false;
  parentUrl: string = '';
  detail: EntryDetail | null = null;

  // error attributes
  error: string | null = null;

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

    if (!environment.production) {
      console.log(`Loading path: ${ path }`)
    }

    this.backend.getIndexFile(path).subscribe(result => {
      const value = result.Ok;
      if (value != null) {
        this.handleEntry(value);
      }
    }, error => {
      const response: ResponseResult<EntryInfo> | null = error.error;
      if (response != null && response.Ok != null) {
        this.handleEntry(response.Ok);
      } else {
        this.name = this.getPathName();
        this.title.setTitle(this.name);
        this.path = this.getPath();
        this.hasParent = this.route.snapshot.url.length > 0;
        this.parentUrl = this.getParentPath();
        this.error = error.statusText;
        this.state = 'error';
      }
    });
  }

  private handleEntry(value: EntryInfo) {
    if (value.name == '') {
      this.name = 'Browse';
      this.title.setTitle('Browse');
      this.hasParent = false;
      this.parentUrl = '';
    } else {
      this.name = value.name;
      this.title.setTitle(value.name);
      this.hasParent = true;
      this.parentUrl = this.getParentPath();
    }

    this.path = value.path_pretty;

    this.detail = value.detail;

    if (value.detail.Directory != null) {
      this.state = 'directory';
    }

    const file = value.detail.File;
    if (file != null) {
      if (file.mime_type.startsWith('video/')) {
        this.state = 'media-file';
      } else {
        this.state = 'file';
      }
    }

    const error = value.detail.Error;
    if (error != null) {
      this.state = 'error';
      this.error = error.error;
    } else {
      this.error = null;
    }
  }

  private getPath(): string {
    const snapshot = this.route.snapshot;

    if (snapshot.routeConfig?.path != '**') {
      return '/';
    } else {
      return '/' + snapshot.url.join('/')
    }
  }

  private getParentPath(): string {
    let url = this.route.snapshot.url;
    if (!environment.production) {
      console.log(`Current Url: [${ url }]`);
    }

    if (url.length < 1) {
      return '/';
    }

    if (url[url.length - 1].path == '') {
      url = url.slice(0, url.length - 2);
    } else {
      url = url.slice(0, url.length - 1);
    }

    if (url.length > 0) {
      return `/${ BROWSE_PATH }/${ url.join('/') }/`;
    } else {
      return `/${ BROWSE_PATH }/`;
    }
  }

  private getPathName(): string {
    let url = this.route.snapshot.url;

    if (url.length < 1) {
      return '/';
    } else if (url[url.length - 1].path == '') {
      return url[url.length - 2].path;
    } else {
      return url[url.length - 1].path;
    }
  }
}
