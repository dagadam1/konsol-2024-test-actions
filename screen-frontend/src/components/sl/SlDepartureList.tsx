import {SlData} from "../../util/sl/sl.ts";
import SlDepartureCard from "./SlDepartureCard.tsx";
import React from "react";

const SlDepartureList: React.FC<{sl_data: SlData}> = ({sl_data}) => {
    return <div className="sl-departure-list">
        {sl_data.departures.map(departure => <SlDepartureCard departure={departure}/>)}
    </div>;
};

export default SlDepartureList;
