import { z } from "zod";

import {SlTrackedLine, SlTrackedSite, SlTransportMode, SlLineGroup, SlDepartureState}
    from "./sl-types.ts";

interface SlDeparture {                     // Example:
    site_id: number,                        // 9204
    stop_point_id: number,                  // 2221
    transport_mode: SlTransportMode,        // "METRO"
    line_group: SlLineGroup,                // "Tunnelbanans röda linje"
    line_id: number,                        // 14
    direction_code: number,                 // 1
    journey_id: number,                     // 2025021320171

    stop_point_name: string,                // "Tekniska högskolan"
    stop_point_designation: string,         // "1"
    destination: string,                    // "Mörby centrum"
    via: string | undefined,                // undefined
    direction: string,                      // "Mörby centrum"
    line_designation: string,               // "14"

    scheduled_time: Date,
    expected_time: Date,
    state: SlDepartureState,                // "ATSTOP"
    display_time: string                    // "Nu"
}

const departure_schema = z.object({
    destination: z.string(),
    via: z.optional(z.string()),
    direction_code: z.number(),
    direction: z.string(),
    state: z.nativeEnum(SlDepartureState),
    display: z.string(),
    scheduled: z.string().datetime({ local: true }),
    expected: z.string().datetime({ local: true }),
    journey: z.object({
        id: z.number()
    }),
    stop_point: z.object({
        id: z.number(),
        name: z.string(),
        designation: z.optional(z.string()),
    }),
    line: z.object({
        id: z.number(),
        designation: z.string(),
        transport_mode: z.nativeEnum(SlTransportMode),
        group_of_lines: z.optional(z.nativeEnum(SlLineGroup)),
    })
});

/**
 * Parses a JSON object fetched via the SL API into an {@link SlDeparture} object
 */
function parse_departure(json: object, site_id: number): SlDeparture {
    const parse_result = departure_schema.safeParse(json);
    if (parse_result.success) {
        const data = parse_result.data;
        let line_group = data.line.group_of_lines;
        if (line_group == undefined) {
            if (data.line.transport_mode == SlTransportMode.Bus)
                line_group = SlLineGroup.Bus;
            else
                throw new Error("Expected line group");
        }
        return {
            site_id,
            stop_point_id: data.stop_point.id,
            transport_mode: data.line.transport_mode,
            line_group,
            line_id: data.line.id,
            direction_code: data.direction_code,
            journey_id: data.journey.id,

            stop_point_name: data.stop_point.name,
            stop_point_designation: data.stop_point.designation || "",
            destination: data.destination,
            via: data.via,
            direction: data.direction,
            line_designation: data.line.designation,

            scheduled_time: new Date(data.scheduled),
            expected_time: new Date(data.expected),
            state: data.state,
            display_time: data.display
        };
    } else {
        console.error(json);
        throw parse_result.error;
    }
}

function parse_departures(json: object, site_id: number): SlDeparture[] {
    if (!("departures" in json) || !Array.isArray(json.departures))
        throw new Error("Expected JSON property 'departures'");
    return json.departures.map((d: object) => {
        return parse_departure(d, site_id);
    });
}

export type {SlTrackedLine, SlTrackedSite, SlDepartureState, SlDeparture};
export {SlTransportMode, SlLineGroup, parse_departures}
