import {
	ActionStateContext,
	actionStateReducer,
	createDefaultActionState,
	DispatchActionStateContext,
} from './action.hooks.ts';
import { PropsWithChildren, useReducer } from 'react';

export function ActionStateProvider({ children }: Readonly<PropsWithChildren>) {
	const [actionState, dispatch] = useReducer(actionStateReducer, createDefaultActionState());

	return (
		<DispatchActionStateContext.Provider value={dispatch}>
			<ActionStateContext.Provider value={actionState}>{children}</ActionStateContext.Provider>
		</DispatchActionStateContext.Provider>
	);
}
