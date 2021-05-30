import { Component, OnInit } from '@angular/core';
import { Observable } from "rxjs";
import { Status } from "../backend.types";
import { map } from "rxjs/operators";
import { BackendService } from "../backend.service";
import { Title } from "@angular/platform-browser";

@Component({
  selector: 'app-welcome',
  templateUrl: './welcome.component.html',
  styleUrls: ['./welcome.component.scss']
})
export class WelcomeComponent implements OnInit {

  status$: Observable<Status | null> = this.backend.status$.pipe(
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
