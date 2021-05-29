/// Encapsulates all API responses coming from the server.
export interface ResponseResult<T> {
  Ok: T | null;
  Err: string | null;
}

export interface Status {
  name: string,
  version: string,
}
