import { Component, HostListener } from '@angular/core';
import { AnchorService } from "./anchor.service";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  constructor(private anchorService: AnchorService) {}

  @HostListener('document:click', ['$event'])
  onDocumentClick(event: Event) {
    // Attempt to use the router on all link clicks so as to not unnecessarily reload the page.
    this.anchorService.interceptClick(event);
  }
}
