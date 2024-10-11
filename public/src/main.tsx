import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';

import './index.scss';
import {App} from './app.tsx';
import {APIProvider} from './providers/api.tsx';
import axios from 'axios';
import {ErrorBoundary, ErrorScreen} from './screens/error.tsx';
import {GameProvider} from './providers/game.tsx';
import {ActionStateProvider} from './providers/action.tsx';

axios.defaults.baseURL = 'http://localhost:8080/';

const root = createRoot(document.getElementById('root')!);
root.render(
    <StrictMode>
        <ErrorBoundary fallback={(error) => <ErrorScreen error={error}/>}>
            <ActionStateProvider>
                <APIProvider>
                    <GameProvider>
                        <App/>
                    </GameProvider>
                </APIProvider>
            </ActionStateProvider>
        </ErrorBoundary>
    </StrictMode>,
);
