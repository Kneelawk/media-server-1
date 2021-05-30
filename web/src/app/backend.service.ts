import { Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { EntryInfo, ResponseResult, Status } from "./backend.types";
import { environment } from "../environments/environment";
import { Observable } from "rxjs";
import { share } from "rxjs/operators";

@Injectable({
  providedIn: 'root'
})
export class BackendService {

  private static baseUrl = environment.serve ? 'http://localhost:9090' : '';
  private static apiUrl = `${ BackendService.baseUrl }/api/v1`
  private static statusUrl = `${ BackendService.apiUrl }/status`
  private static indexFilesUrl = `${ BackendService.apiUrl }/index/files`

  status$: Observable<ResponseResult<Status>> = this.getStatus().pipe(share());

  constructor(private client: HttpClient) {
    console.log('Loading server details...')
    this.status$.subscribe(value => {
      if (value.Ok != null) {
        console.log(`Server-name:    ${ value.Ok.name }`);
        console.log(`Server-version: ${ value.Ok.version }`);
      } else if (value.Err != null) {
        console.log(`Error getting server status: ${ value.Err }`);
      }
    })
  }

  getIndexFile(path: string): Observable<ResponseResult<EntryInfo>> {
    return this.client.get<ResponseResult<EntryInfo>>(`${ BackendService.indexFilesUrl }${ path }`)
  }

  private getStatus(): Observable<ResponseResult<Status>> {
    return this.client.get<ResponseResult<Status>>(BackendService.statusUrl);
  }
}
