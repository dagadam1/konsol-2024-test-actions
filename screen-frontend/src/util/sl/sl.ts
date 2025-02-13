import {
    SlTrackedSite,
    SlDeparture,
    parse_departure
} from "./types.ts";

function parse_departures(json: object, site_id: number): SlDeparture[] {
    if (!("departures" in json) || !Array.isArray(json.departures))
        throw new Error("Expected JSON property 'departures'");
    return json.departures.map((d: object) => {
        return parse_departure(d, site_id);
    });
}

class SlData {
    tracked_sites: SlTrackedSite[];
    departures: SlDeparture[];
    last_update: Date | null; // null if no update has taken place yet

    constructor(tracked_sites: SlTrackedSite[]) {
        this.tracked_sites = tracked_sites;
        this.departures = [];
        this.last_update = null;
    }

    async update() {
        const departures: SlDeparture[] = [];

        for (const tracked_site of this.tracked_sites) {
            const url = `https://transport.integration.sl.se/v1/sites/${tracked_site.site_id}/departures`;
            const response = await fetch(url);
            if (!response.ok) {
                console.error(`Failed to fetch data from site with id ${tracked_site.site_id}`);
                return;
            }
            const json: object = await response.json();
            let unfiltered_departures;
            try {
                unfiltered_departures = parse_departures(json, tracked_site.site_id);
            } catch (e) {
                console.error(e);
                return;
            }
            const filtered_departures = unfiltered_departures.filter(departure =>
                tracked_site.tracked_lines === undefined ||
                tracked_site.tracked_lines.some(tracked_line =>
                    tracked_line.transport_mode === departure.transport_mode &&
                    tracked_line.line_id === departure.line_id &&
                    (tracked_line.direction_code === undefined || tracked_line.direction_code == departure.direction_code)
                )
            );
            departures.push(...filtered_departures);
        }

        this.departures = departures;
        this.last_update = new Date();
    }
}

export { SlData };