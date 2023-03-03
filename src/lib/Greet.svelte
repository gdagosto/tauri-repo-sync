<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Repository } from '$lib/types';

	import { appWindow } from '@tauri-apps/api/window';
	appWindow.listen('try_to_pull_command', ({ payload }) => {
		console.log('try_to_pull_command', payload);
	});

	export let data: Repository;

	async function checkForUpdates() {
		invoke('check_for_updates', { data })
			.then((res) => {
				// Checa se deveria dar pull
				if (res === 'shouldPull') {
					console.log('shouldPull');
				} else {
					console.log(res);
				}
			})
			.catch((e) => console.error(e));
	}

	async function tryToPull() {
		invoke('try_to_pull', { data, commands: ['yarn build'] })
			.then((res) => console.log(res))
			.catch((e) => console.error(e));
	}
</script>

<div class="wrapper">
	<div class="buttons">
		<button on:click={checkForUpdates}>Check for updates</button>
		<button on:click={tryToPull}>Try to pull</button>
	</div>
</div>

<style lang="scss">
	.wrapper {
		display: flex;
		flex-direction: column;
		row-gap: 2rem;
		align-items: center;

		background-color: #ddd;
		padding: 2rem;
		border-radius: 0.5rem;
	}

	form {
		display: grid;
		grid-template-columns: auto 2fr;
		column-gap: 0.5rem;
		row-gap: 2rem;
		align-items: baseline;

		label {
			text-align: right;
			font-size: 0.875rem;
		}

		input {
			min-width: 250px;
		}
	}
	.buttons {
		width: 100%;
		display: flex;
		flex-direction: column;
		row-gap: 0.5rem;
		justify-content: space-between;
	}
</style>
