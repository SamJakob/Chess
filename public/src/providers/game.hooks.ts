import { createContext, useContext } from 'react';
import { AsyncValue, RefreshFunction } from '../utilities/async.ts';
import { Game } from '../api/game.ts';

export const RefreshGameContext = createContext<RefreshFunction>(undefined as never);
export const GameContext = createContext<AsyncValue<Game>>(undefined as never);

export const useRefreshGame = () => useContext(RefreshGameContext);
export const useGame = () => useContext(GameContext);
