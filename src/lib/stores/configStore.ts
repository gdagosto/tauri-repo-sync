import { writable } from 'svelte/store';
import { writeTextFile, createDir, exists, BaseDirectory, readTextFile } from '@tauri-apps/api/fs';
import type { Repository } from '$lib/types';

const defaultValue: Repository[] = [];
const opts = { dir: BaseDirectory.AppData };
const FILE_PATH = 'config\\test.json';

function createConfigStore() {
	const { subscribe, set, update } = writable(defaultValue);

	return {
		subscribe,
		set,
		update
	};
}

async function loadConfigStoreData() {
	// Check if file exists
	const fileExists = await exists(FILE_PATH, opts);
	if (!fileExists) {
		// Check if folder exists
		const folderExists = await exists('config', opts);
		if (!folderExists) {
			await createDir('config', { ...opts, recursive: true });
		}

		await writeTextFile(FILE_PATH, '', opts);
	} else {
		const fileData = JSON.parse(await readTextFile(FILE_PATH, opts));
		stConfig.set(fileData);
	}
}

const stConfig = createConfigStore();
loadConfigStoreData();

stConfig.subscribe(($stConfig) => {
	// Whenever $stConfig changes, save to the text file
	writeTextFile(FILE_PATH, JSON.stringify($stConfig), opts);
});

export { stConfig };
