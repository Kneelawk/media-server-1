import { Component, OnInit } from '@angular/core';
import { BackendService } from "../backend.service";
import { DirectoryChild } from "../backend.types";

@Component({
  selector: 'app-browse',
  templateUrl: './browse.component.html',
  styleUrls: ['./browse.component.scss']
})
export class BrowseComponent implements OnInit {

  name: string = 'Loading...'
  path: string = 'Loading...'
  children: Array<DirectoryChild> = [];

  constructor(private backend: BackendService) { }

  ngOnInit(): void {
    this.backend.getIndexFile('').subscribe(result => {
      const value = result.Ok;
      if (value != null) {
        if (value.name == '') {
          this.name = 'Browse';
        } else {
          this.name = value.name;
        }

        this.path = value.path_pretty;

        const dir = value.detail.Directory;
        if (dir != null) {
          this.children = dir.children;
        }
      }
    });
  }

}
