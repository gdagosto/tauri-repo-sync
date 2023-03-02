<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { PUBLIC_LOCAL_PATH, PUBLIC_BRANCH_NAME } from '$env/static/public';

	let localPath = PUBLIC_LOCAL_PATH;
	let localBranch = PUBLIC_BRANCH_NAME;
	let remoteName = 'origin';
	let remoteBranch = PUBLIC_BRANCH_NAME;

	function changeRepo(
		local_path: string,
		local_branch: string,
		remote_name: string,
		remote_branch: string
	) {
		return {
			local_path,
			local_branch,
			remote_name,
			remote_branch
		};
	}

	$: data = changeRepo(localPath, localBranch, remoteName, remoteBranch);

	async function checkForUpdates() {
		invoke('check_for_updates', { data })
			.then((res) => {
				// Checa se deveria dar pull
				if (res === 'shouldPull') {
					console.log('shouldPull');
				}
			})
			.catch((e) => console.error(e));
	}

	async function tryToPull() {
		invoke('try_to_pull', { data })
			.then((res) => console.log(res))
			.catch((e) => console.error(e));
	}
</script>

<div class="wrapper">
	<form action="">
		<label for="inpLocalPath">Local path</label>
		<input type="text" id="inpLocalPath" bind:value={localPath} />
		<label for="inpLocalBranch">Local branch</label>
		<input type="text" id="inpLocalBranch" bind:value={localBranch} />
		<label for="inpRemoteName">Remote name</label>
		<input type="text" id="inpRemoteName" bind:value={remoteName} />
		<label for="inpRemoteBranch">Remote branch</label>
		<input type="text" id="inpRemoteBranch" bind:value={remoteBranch} />
	</form>

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
