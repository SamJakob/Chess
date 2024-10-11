import {createContext, useContext} from 'react';

export interface APIDetails {
    version: string;
}

export const APIDetailsContext = createContext<APIDetails | undefined>(undefined);

export const useAPIDetails = () => useContext(APIDetailsContext);
