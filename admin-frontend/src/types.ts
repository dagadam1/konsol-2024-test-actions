interface SlideData {
    id: string;
    caption: string;
    start_date: Date;
    end_date: Date;
    active: boolean;
    filetype: string;
}

interface UserData {
    id: string;
    email: string;
    admin: boolean;
}

export type { SlideData, UserData };