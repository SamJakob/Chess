import { APIContext, APIDetails } from './api.hooks.ts';
import { PropsWithChildren, useMemo } from 'react';
import { useAsync } from '../utilities/async.ts';
import { getAPIVersion } from '../api/version.ts';

export function APIProvider({ children }: Readonly<PropsWithChildren>) {
	const [version, _] = useAsync(getAPIVersion);

	const apiContext = useMemo(() => {
		if (version.state !== 'data') {
			return undefined;
		}
		return { version: version.data } as APIDetails;
	}, [version]);

	return <APIContext.Provider value={apiContext}>{children}</APIContext.Provider>;
}
