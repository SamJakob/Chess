/// <reference types="vite-plugin-svgr/client" />

import {useCallback, useEffect, useRef, useState} from 'react';
import './piece.scss';

import king from '../assets/king.svg?react';
import queen from '../assets/queen.svg?react';
import knight from '../assets/knight.svg?react';
import rook from '../assets/rook.svg?react';
import bishop from '../assets/bishop.svg?react';
import pawn from '../assets/pawn.svg?react';

import kingDark from '../assets/king-dark.svg?react';
import queenDark from '../assets/queen-dark.svg?react';
import knightDark from '../assets/knight-dark.svg?react';
import rookDark from '../assets/rook-dark.svg?react';
import bishopDark from '../assets/bishop-dark.svg?react';
import pawnDark from '../assets/pawn-dark.svg?react';
import {useActionState, useDispatchActionState} from '../providers/action.hooks.ts';
import {useGame, useRefreshGame} from '../providers/game.hooks.ts';
import {move} from '../api/move.ts';
import {requireAsync} from '../utilities/async.ts';

const pieces = {
    W: {
        K: king,
        Q: queen,
        N: knight,
        R: rook,
        B: bishop,
        P: pawn,
    },
    B: {
        K: kingDark,
        Q: queenDark,
        N: knightDark,
        R: rookDark,
        B: bishopDark,
        P: pawnDark,
    },
} as const;

export type Color = keyof typeof pieces;
export type Kind = keyof (typeof pieces)[Color];

export interface ChessPieceProps {
    kind: Kind;
    color: Color;
    position: [number, number]; // [rank, file]
}

export function ChessPiece({kind, color, position}: Readonly<ChessPieceProps>) {
    const Piece = pieces[color][kind];
    const wrapper = useRef(null);

    const actionState = useActionState();
    const dispatchActionState = useDispatchActionState();

    const game = requireAsync(useGame());
    const refreshGame = useRefreshGame();

    const [isDragging, setIsDragging] = useState(false);
    const [offset, setOffset] = useState<[number, number] | undefined>(undefined);
    const [location, setLocation] = useState<[number, number] | undefined>(undefined);

    function getWrapperTarget(target: EventTarget | null) {
        if (wrapper.current !== null && target && target instanceof Element) {
            let parent: Element | null = target;

            while (parent && parent !== wrapper.current) {
                parent = parent.parentElement;
            }

            if (parent === wrapper.current) return parent;
        }
    }

    const startDrag = useCallback(
        (e: MouseEvent) => {
            if (actionState.isDragging || actionState.isLoading) {
                return;
            }

            const target = getWrapperTarget(e.target);
            if (!target) return;

            setOffset([e.clientX, e.clientY]);
            setLocation([0, 0]);
            dispatchActionState({
                action: 'drag',
            });

            setIsDragging(true);
        },
        [dispatchActionState, actionState.isDragging, actionState.isLoading],
    );

    const endDrag = useCallback(() => {
        if (isDragging) {
            setOffset(undefined);
            setLocation(undefined);
            setIsDragging(false);

            dispatchActionState({
                action: 'endDrag',
            });

            const hoveredSquare = document.querySelector('.square.hovered');

            let rank;
            let file;

            if (hoveredSquare) {
                hoveredSquare.classList.remove('hovered');

                const rawRank = hoveredSquare.getAttribute('data-rank');
                const rawFile = hoveredSquare.getAttribute('data-file');

                if (rawRank) rank = parseInt(rawRank);
                if (rawFile) file = parseInt(rawFile);
            }

            (async () => {
                if (rank !== undefined && file !== undefined) {
                    await move(game, position, [rank, file]);
                }

                await refreshGame();
                dispatchActionState({
                    action: 'ready',
                });
            })().catch(() => {
                (async () => {
                    try {
                        await refreshGame();

                        dispatchActionState({
                            action: 'ready',
                        });
                    } catch (error) {
                        dispatchActionState({
                            action: 'error',
                            error,
                        });
                    }
                })();
            });
        }
    }, [isDragging, dispatchActionState, game, position, refreshGame]);

    const continueDrag = useCallback(
        (e: MouseEvent) => {
            if (!isDragging) return;

            // Get the square that the mouse is currently over.
            const squares = document.querySelectorAll('.square');
            squares.forEach((square) => {
                const boundingBox = square.getBoundingClientRect();
                if (
                    boundingBox.left <= e.clientX &&
                    boundingBox.right >= e.clientX &&
                    boundingBox.top <= e.clientY &&
                    boundingBox.bottom >= e.clientY
                ) {
                    square.classList.add('hovered');
                } else {
                    square.classList.remove('hovered');
                }
            });

            setLocation((location) => [
                e.clientX - offset![0] + location![0], //
                e.clientY - offset![1] + location![1], //
            ]);
            setOffset([e.clientX, e.clientY]);
        },
        [isDragging, offset],
    );

    useEffect(() => {
        document.addEventListener('mousedown', startDrag);
        return () => document.removeEventListener('mousedown', startDrag);
    }, [startDrag]);

    useEffect(() => {
        document.addEventListener('mouseup', endDrag);
        return () => document.removeEventListener('mouseup', endDrag);
    }, [endDrag]);

    useEffect(() => {
        document.addEventListener('mousemove', continueDrag);
        return () => document.removeEventListener('mousemove', continueDrag);
    }, [continueDrag]);

    return (
        <span
            ref={wrapper}
            className={[
                'chess-piece-wrapper',
                isDragging ? 'is-dragging' : undefined,
                actionState.isDragging ? 'is-other-dragging' : undefined,
                actionState.isLoading ? 'is-loading' : undefined,
            ]
                .filter((x) => x)
                .join(' ')}
            style={{
                left: `${location !== undefined ? location[0] : 0}px`,
                top: `${location !== undefined ? location[1] : 0}px`,
            }}>
			<Piece title={`${color} ${kind}`} className={['chess-piece', color === 'B' ? 'dark' : 'light'].join(' ')}/>
		</span>
    );
}
