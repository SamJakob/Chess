import { MainScreen } from './screens/main.tsx';
import { useAPIContext } from './providers/api.hooks.ts';
import { LoadingScreen } from './screens/loading.tsx';
import { useGame } from './providers/game.hooks.ts';
import { useActionState } from './providers/action.hooks.ts';
import { ErrorScreen } from './screens/error.tsx';

export function App() {
	const api = useAPIContext();
	const game = useGame();
	const actionState = useActionState();

	if (actionState.error !== undefined) {
		return <ErrorScreen error={actionState.error} />;
	}

	if (api === undefined || game.state === 'loading') {
		return <LoadingScreen />;
	} else {
		return <MainScreen />;
	}
}
