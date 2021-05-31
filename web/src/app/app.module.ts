import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { MenuBarComponent } from './menu-bar/menu-bar.component';
import { HttpClientModule } from "@angular/common/http";
import { WelcomeComponent } from './welcome/welcome.component';
import { BrowseComponent } from './browse/browse.component';
import { MarkdownModule, MarkedOptions } from "ngx-markdown";
import { markedOptionsFactory } from "./markdown-util";
import { AnchorService } from "./anchor.service";
import { BrowseErrorComponent } from './browse-error/browse-error.component';
import { BrowseDirectoryComponent } from './browse-directory/browse-directory.component';
import { BrowseMediaFileComponent } from './browse-media-file/browse-media-file.component';

@NgModule({
  declarations: [
    AppComponent,
    MenuBarComponent,
    WelcomeComponent,
    BrowseComponent,
    BrowseErrorComponent,
    BrowseDirectoryComponent,
    BrowseMediaFileComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    MarkdownModule.forRoot({
      markedOptions: {
        provide: MarkedOptions,
        useFactory: markedOptionsFactory,
        deps: [AnchorService]
      }
    })
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {}
