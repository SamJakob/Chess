import { createContext, useContext } from 'react';

export interface APIDetails {
	version: string;
}

export const APIContext = createContext<APIDetails | undefined>(undefined);

export const useAPIContext = () => useContext(APIContext);
