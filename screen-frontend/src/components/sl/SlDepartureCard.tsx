import React from "react";
import {SlDeparture} from "../../util/sl/types.ts";
import "../../styles/SlDepartureCard.css";
import SlLineBadge from "./SlLineBadge.tsx";

const SlDepartureCard: React.FC<{ departure: SlDeparture }> = ({ departure }) => {
    // == components ==
    // stop point name
    // transport mode
    // line group
    // line designation
    // line destination
    // via
    //
    // display time
    // stop point designation

    return (
        <div className="sl-departure">
            <div className="sl-departure-left">
                <p className="sl-stop-point-name">{"från: " + departure.stop_point_name}</p>
                <p className="sl-destination">{"till: " + departure.destination + (departure.via ? ` (via ${departure.via})` : "")}</p>
            </div>
            <div className="sl-departure-middle">
                <SlLineBadge mode={departure.transport_mode} line_group={departure.line_group} line_designation={departure.line_designation}/>
            </div>
            <div className="sl-departure-middle-right">
                <p className="sl-display-time large-text">{departure.display_time}</p>
            </div>
            <div className="sl-departure-right">
                <p className="sl-stop-point large-text">{departure.stop_point_designation}</p>
            </div>
        </div>
    )
};

export default SlDepartureCard;