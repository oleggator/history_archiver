<script>
	import Day from './Day.svelte';
	import { query } from '../api.js';

	let query_input = '';
	$: promise = query(query_input, 1000);
</script>

<div class="header">
	<div class="bar" >
		<div class="title">History</div>

		<input class="searchbar" type="text" placeholder="Search..." bind:value={query_input}>
	</div>
</div>
<div class="days">
	{#await promise}
		<div class="loading-placeholder">
			Loading...
		</div>
	{:then days}
		{#each Object.entries(days) as [date, visits]}
			<Day date={date} visits={visits} />
		{/each}
	{/await}
</div>

<style>
	.header {
		/* position: sticky; */
		/* top: 0; */
		padding: 8px;
	}

	.bar {
		display: flex;
		justify-content: space-between;

		padding: 8px;

		background-color: #B4CFB0;
		border-radius: 8px;
	}

	.bar > .title {
		font-weight: bolder;
		font-size: xx-large;
	}

	.bar .searchbar {
		border-radius: 8px;
		margin: 0;
	}

	.days .loading-placeholder {
		font-weight: bolder;
		font-size: xx-large;
		text-align: center;
	}
</style>
