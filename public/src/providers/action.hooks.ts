import { createContext, Dispatch, useContext } from 'react';
import { produce } from 'immer';

export type Action =
	| {
			action: 'drag' | 'endDrag' | 'ready';
	  }
	| {
			action: 'error';
			error: unknown;
	  };

export interface ActionState {
	error?: unknown;
	isDragging: boolean;
	isLoading: boolean;
}

export const DispatchActionStateContext = createContext<Dispatch<Action>>(undefined as never);
export const ActionStateContext = createContext<ActionState>(undefined as never);

export const useDispatchActionState = () => useContext(DispatchActionStateContext);
export const useActionState = () => useContext(ActionStateContext);

export const createDefaultActionState = (): ActionState => ({
	error: undefined,
	isDragging: false,
	isLoading: false,
});

export const actionStateReducer = produce((draft, action: Action) => {
	switch (action.action) {
		case 'drag': {
			draft.isDragging = true;
			break;
		}
		case 'endDrag': {
			draft.isDragging = false;
			draft.isLoading = true;
			break;
		}
		case 'ready': {
			draft.isDragging = false;
			draft.isLoading = false;
			break;
		}
		case 'error': {
			draft.error = action.error;
			break;
		}
	}
});
