import { useCallback, useEffect, useRef, useState } from 'react';

const asyncValue = Symbol('asyncValue');

export interface AsyncLoading {
	[asyncValue]: true;
	state: 'loading';
}

export interface AsyncError {
	[asyncValue]: true;
	state: 'error';
	error: unknown;
}

export interface AsyncData<T> {
	[asyncValue]: true;
	state: 'data';
	data: T;
}

export type AsyncValue<T> = AsyncLoading | AsyncError | AsyncData<T>;
export type RefreshFunction = () => Promise<void>;

export function useAsync<T>(
	promise: () => Promise<T>,
	options?: { loadOnRefresh?: boolean; allowError?: boolean; refreshAction?: (previous: T) => Promise<T> },
) {
	const [state, setState] = useState<AsyncValue<T>>({
		[asyncValue]: true,
		state: 'loading',
	});

	const initialRequestMade = useRef(false);
	const previousData = useRef<T | undefined>(undefined);
	const loadOnRefresh = options?.loadOnRefresh;
	const refreshAction = options?.refreshAction;

	const refresh = useCallback(async () => {
		if (initialRequestMade.current && !previousData.current) {
			return;
		}
		initialRequestMade.current = true;

		if (loadOnRefresh) {
			setState({
				[asyncValue]: true,
				state: 'loading',
			});
		}

		try {
			const data =
				previousData.current && refreshAction ? await refreshAction(previousData.current) : await promise();

			setState({
				[asyncValue]: true,
				state: 'data',
				data,
			});

			previousData.current = data;
		} catch (error) {
			setState({
				[asyncValue]: true,
				state: 'error',
				error,
			});
		}
	}, [loadOnRefresh, refreshAction, promise]);

	useEffect(() => {
		void (async () => {
			await refresh();
		})();
	}, [refresh]);

	if (state.state === 'error' && !options?.allowError) {
		throw state.error;
	}

	return [state, refresh as RefreshFunction] as const;
}

export function requireAsync<T>(value: AsyncValue<T>): T {
	if (value.state === 'loading') throw new Error('value was unexpectedly still loading');
	if (value.state === 'error') throw value.error;
	return value.data;
}
