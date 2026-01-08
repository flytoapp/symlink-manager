// Item status enum
export type ItemStatus = 'active' | 'inactive' | 'broken' | 'conflict';

// Single item in a source directory
export interface Item {
  name: string;
  isDirectory: boolean;
  sourcePath: string;
  status: ItemStatus;
  enabled: boolean;
  conflictSource?: string;
}

// Source configuration
export interface Source {
  id: string;
  name: string;
  sourcePath: string;
  targetPath?: string;
  enabledItems: string[];
}

// Profile configuration
export interface Profile {
  id: string;
  name: string;
  basePath: string;
  sources: Source[];
}

// Root configuration
export interface AppConfig {
  version: number;
  profiles: Profile[];
  activeProfileId?: string;
}

// Command response types
export interface SymlinkResult {
  success: boolean;
  itemName: string;
  error?: string;
}

export interface PermissionStatus {
  canCreateSymlinks: boolean;
  requiresElevation: boolean;
  isDeveloperMode: boolean;
  errorMessage?: string;
}
