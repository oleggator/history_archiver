<script>
    import { fade } from 'svelte/transition';
    import Visit from "./Visit.svelte";

    export let visits;
    export let date;
</script>

<div transition:fade class="day">
    <div class="day-header">{date}</div>
    <div class="day-body">
        {#each visits as visit}
            <Visit
                time={visit.visit_time.toLocaleTimeString()}
                title={visit.title}
                url={visit.url}
            />
        {/each}
    </div>
</div>

<style>
    .day-header {
        position: sticky;
        top: 0;
        font-weight: bold;
        background-color: #B4CFB0;
        padding: 8px;
		/* margin: 8px; */
    }

    .day {
        border-radius: 8px;

        padding: 8px;
		margin: 8px;

        background-color: #B4CFB0;
    }
</style>
