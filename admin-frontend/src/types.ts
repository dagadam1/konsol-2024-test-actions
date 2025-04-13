import { Dispatch, SetStateAction } from "react";

interface SlideData {
    id: string;
    caption: string;
    start_date: Date;
    end_date: Date;
    active: boolean;
    filetype: string;
}

interface User {
    email: string;
}


// type UserContextType = User | null;
type UserContextType = {
    user: User | null;
    setUser: Dispatch<SetStateAction<User | null>>;
};

export type { SlideData, User, UserContextType };