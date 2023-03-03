export interface Repository {
	local_path: string;
	local_branch: string;
	remote_name: string;
	remote_branch: string;
	commands: string[];
}
