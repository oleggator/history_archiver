<script>
    export let time;
    export let title;
    export let url;
</script>
<a href={url} title={url}>
    <div class="visit">
        <span class="time">{time}</span>
        {title}
    </div>
</a>


<style>
    .time {
        color: #362706;
        font-weight: bold;
    }
    .visit {
        color: #E5E3C9;
        background-color: #94B49F;

        padding: 5px;
        margin: 5px;

        border-radius: 5px;
    }
</style>

