<script>
	import Greet from '$lib/Greet.svelte';
	import { stConfig } from '$lib/stores';
	import { PUBLIC_LOCAL_PATH, PUBLIC_BRANCH_NAME } from '$env/static/public';
	import Repository from '$lib/components/Repository.svelte';

	$: console.log($stConfig);

	stConfig.set([
		{
			local_path: PUBLIC_LOCAL_PATH,
			local_branch: PUBLIC_BRANCH_NAME,
			remote_name: 'origin',
			remote_branch: PUBLIC_BRANCH_NAME,
			commands: []
		}
	]);
</script>

<div class="wrapper">
	<div class="repositoryList">
		{#each $stConfig as repo}
			<Repository data={repo} />
		{/each}
	</div>

	<Greet data={$stConfig[0]} />
</div>

<style lang="scss">
	.wrapper {
		width: 100%;
		min-height: 95vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.repositoryList {
	}
</style>
