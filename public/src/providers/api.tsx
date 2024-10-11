import {APIDetailsContext} from './api.hooks.ts';
import {PropsWithChildren} from 'react';
import {useAsync} from '../utilities/async.ts';
import {getAPIDetails} from '../api/version.ts';

export function APIProvider({children}: Readonly<PropsWithChildren>) {
    const [details, _] = useAsync(getAPIDetails);

    return (
        <APIDetailsContext.Provider value={details.state === 'data' ? details.data : undefined}>
            {children}
        </APIDetailsContext.Provider>
    );
}
