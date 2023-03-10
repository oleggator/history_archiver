<script>
	import History from './components/History.svelte';
</script>

<History/>

<style>
	:global(*) {
		margin: 0;
		padding: 0;

		box-sizing: border-box;
	}
	:global(body), :global(html) {
		overflow: auto;

		position: absolute;

		width: 100%;
		height: 100%;

		margin: 0;
		padding: 0;
	}
</style>
