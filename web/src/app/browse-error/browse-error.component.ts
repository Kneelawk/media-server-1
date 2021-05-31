import { Component, Input, OnInit } from '@angular/core';
import { EntryDetailError } from "../backend.types";

@Component({
  selector: 'app-browse-error',
  templateUrl: './browse-error.component.html',
  styleUrls: ['./browse-error.component.scss']
})
export class BrowseErrorComponent {
  @Input()
  error: string | null | undefined;
}
