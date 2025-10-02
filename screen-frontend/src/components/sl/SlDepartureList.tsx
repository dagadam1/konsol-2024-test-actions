import React from "react";

import SlData from "../../types/sl/SlData.ts";
import SlDepartureCard from "./SlDepartureCard.tsx";

const SlDepartureList: React.FC<{sl_data: SlData}> = ({sl_data}) => {
    return <div className="sl-departure-list">
        {sl_data.departures.map(departure => <SlDepartureCard departure={departure}/>)}
    </div>;
};

export default SlDepartureList;
