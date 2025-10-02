enum SlTransportMode {
    Train = "TRAIN",    // Pendeltåg
    Metro = "METRO",    // Röda, gröna, blå linjen
    Bus = "BUS",        // Röd buss, blå buss
    Tram = "TRAM",      // Spårväg city, Roslagsbanan etc.
    Ferry = "FERRY",
    Ship = "SHIP",
    Taxi = "TAXI",
}

enum SlLineGroup {
    Train = "Pendeltåg",
    RedMetro = "Tunnelbanans röda linje",
    GreenMetro = "Tunnelbanans gröna linje",
    BlueMetro = "Tunnelbanans blå linje",
    Bus = "Buss",
    BlueBus = "Blåbuss",
    CityLine = "Spårväg City",
    RoslagenLine = "Roslagsbanan",
}

interface SlTrackedLine {
    transport_mode: SlTransportMode,
    line_id: number,
    direction_code: number | undefined, // if undefined, track all directions
}

interface SlTrackedSite {
    site_id: number, // can be found via https://www.trafiklab.se/api/trafiklab-apis/sl/stop-lookup (generally last 4 digits of SiteId)
    tracked_lines: SlTrackedLine[] | undefined, // if undefined, track all departures from a given site
}

enum SlDepartureState {
    AtStop = "ATSTOP",
    Expected = "EXPECTED",
    NotExpected = "NOTEXPECTED",
    Replaced = "REPLACED",
    Cancelled = "CANCELLED",
    Missed = "MISSED",
    Passed = "PASSED",
    NotCalled = "NOTCALLED",
    Inhibited = "INHIBITED",
    Boarding = "BOARDING",
    BoardingClosed = "BOARDINGCLOSED",
    Departed = "DEPARTED",
    AssumedDeparted = "ASSUMEDDEPARTED",
}

export type {SlTrackedLine, SlTrackedSite};
export {SlTransportMode, SlLineGroup, SlDepartureState};
