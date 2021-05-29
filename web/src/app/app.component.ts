import { Component, OnInit } from '@angular/core';
import { BackendService } from "./backend.service";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";
import { Status } from "./backend.types";
import { Title } from "@angular/platform-browser";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.styl']
})
export class AppComponent implements OnInit {
  status$: Observable<Status | null> = this.backend.getStatus().pipe(
    map(result => result.Ok)
  );
  title$: Observable<string> = this.status$.pipe(
    map(status => status?.welcome_title ?? 'Error loading title')
  );
  content$: Observable<string> = this.status$.pipe(
    map(status => status?.welcome_content ?? 'Error loading content')
  );

  constructor(private backend: BackendService, private title: Title) {}

  ngOnInit(): void {
    this.title$.subscribe(str => this.title.setTitle(str))
  }
}
