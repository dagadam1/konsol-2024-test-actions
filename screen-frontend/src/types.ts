import { Dispatch, SetStateAction } from "react";

interface SlideData {
    id: string;
    caption: string;
    start_date: Date;
    end_date: Date;
    active: boolean;
    filetype: string;
}


export type { SlideData };