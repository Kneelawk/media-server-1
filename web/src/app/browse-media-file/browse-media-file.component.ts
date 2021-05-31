import { Component, ElementRef, HostListener, Input, ViewChild } from '@angular/core';
import { BackendService } from "../backend.service";
import { EntryDetailFile } from "../backend.types";

// There is not really a good way to determine a video's fps so we just assume every video has a *constant* fps of 30.
const fps = 30;

@Component({
  selector: 'app-browse-media-file',
  templateUrl: './browse-media-file.component.html',
  styleUrls: ['./browse-media-file.component.scss']
})
export class BrowseMediaFileComponent {

  @Input()
  file!: EntryDetailFile;

  @ViewChild('player')
  private playerRef: ElementRef | undefined;

  @HostListener('document:keypress', ['$event'])
  onDocumentKeyPress(event: KeyboardEvent) {
    if (this.playerRef) {
      const player: HTMLVideoElement = this.playerRef.nativeElement;

      let code = event.key;
      if (code === 'k' || code === 'K') {
        if (player.paused || player.ended) {
          player.play();
        } else {
          player.pause();
        }
      }
      if (code === 'j' || code === 'J') {
        player.currentTime -= 10;
      }
      if (code === 'l' || code === 'L') {
        player.currentTime += 10;
      }
      if (code === 'h' || code === 'H') {
        player.currentTime -= 5;
      }
      if (code === ';' || code === ':') {
        player.currentTime += 5;
      }
      if (code === 'm' || code === 'M') {
        player.muted = !player.muted;
      }
      if (code === '-' || code === '_') {
        player.volume -= 0.05;
      }
      if (code === '=' || code === '+') {
        player.volume += 0.05;
      }
      if (code === ',' || code === '<') {
        let frame = player.currentTime * fps;
        frame -= 1;
        player.currentTime = frame / fps + 0.00001;
      }
      if (code === '.' || code === '>') {
        let frame = player.currentTime * fps;
        frame += 1;
        player.currentTime = frame / fps + 0.00001;
      }
    }
  }

  url(url: string): string {
    return BackendService.url(url);
  }
}
