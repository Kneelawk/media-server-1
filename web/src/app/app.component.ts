import { Component } from '@angular/core';
import { BackendService } from "./backend.service";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.styl']
})
export class AppComponent {
  title = 'media-server-one';

  constructor(private backend: BackendService) {}
}
