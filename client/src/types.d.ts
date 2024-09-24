export interface Dirent {
  fileName: string;
  path: string;
  metadata: Metadata;
}

export interface File {
  fileName: string;
}

export interface Metadata {
  fileType: string;
}

export interface Node {
  id: number;
}

export interface Link {
  source: number;
  target: number;
}
