import { Component, Input } from '@angular/core';
import { DirectoryChild, EntryDetailDirectory } from "../backend.types";
import { Router } from "@angular/router";

@Component({
  selector: 'app-browse-directory',
  templateUrl: './browse-directory.component.html',
  styleUrls: ['./browse-directory.component.scss']
})
export class BrowseDirectoryComponent {
  children!: Array<DirectoryChild>;

  constructor(private router: Router) {}

  private _directory!: EntryDetailDirectory;

  @Input()
  get directory(): EntryDetailDirectory {
    return this._directory;
  }

  set directory(directory1: EntryDetailDirectory) {
    this._directory = directory1;
    this.children = directory1.children;
    this.children.sort((a, b) => a.name.localeCompare(b.name));
  }

  navigateTo(path: string) {
    const fullPath = `/tree${ path }`;
    this.router.navigateByUrl(fullPath).then(_ => {});
  }
}
