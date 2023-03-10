const search_url = 'http://127.0.0.1:7700/indexes/visits/search';

export async function query(query, limit) {
    const req = {
        q: query,
        limit: limit ?? 100,
        sort: [
            'visit_time:desc',
        ],
    };
    const resp = await fetch(search_url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(req),
    });
    const { hits } = await resp.json();

    const days = {};
    for (const visit of hits) {
        // const date = Date.parse(visit.visit_time).toDateString();
        const date = new Date(visit.visit_time / 1_000_000);
        const dateString = date.toDateString();
        visit.visit_time = date;

        const day = days[dateString] ?? [];
        day.push(visit);

        days[dateString] = day;
    }

    return days;
}
