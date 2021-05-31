/// Encapsulates all API responses coming from the server.
export interface ResponseResult<T> {
  Ok: T | null;
  Err: string | null;
}

/// Represents the backend server status.
export interface Status {
  name: string;
  version: string;
  welcome_title: string;
  welcome_content: string;
}

/// Represents general file node information.
export interface EntryInfo {
  detail: EntryDetail;
  name: string;
  path: string;
  path_pretty: string;
}

/// Represents more specific file node information.
export interface EntryDetail {
  Directory: EntryDetailDirectory | null;
  Error: EntryDetailError | null;
  File: EntryDetailFile | null;
}

/// Represents a file node that is a directory. This contains directory specific information.
export interface EntryDetailDirectory {
  children: Array<DirectoryChild>;
}

/// Represents a file node that the backend was unable to load, either because it does not exist or because the backend
/// was not permitted to read it.
export interface EntryDetailError {
  error: 'NotFound' | 'Forbidden'
}

/// Represents a file node that is a file. This gives the cdn url where the file can be obtained.
export interface EntryDetailFile {
  mime_type: string;
  url: string;
}

/// Represents a child element inside a directory.
export interface DirectoryChild {
  name: string;
  type: 'Directory' | 'File';
  url: string;
  path: string;
}
