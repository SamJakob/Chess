import { MainScreen } from './screens/main.tsx';
import { useAPIDetails } from './providers/api.hooks.ts';
import { LoadingScreen } from './screens/loading.tsx';
import { useGame } from './providers/game.hooks.ts';
import { useActionState } from './providers/action.hooks.ts';
import { ErrorScreen } from './screens/error.tsx';

export function App() {
	const actionState = useActionState();

	return (
		<div
			className={['app', 'w-full', 'h-full', actionState.isDragging ? 'is-dragging' : undefined]
				.filter((x) => x)
				.join(' ')}>
			<RenderApp />
		</div>
	);
}

function RenderApp() {
	const api = useAPIDetails();
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
