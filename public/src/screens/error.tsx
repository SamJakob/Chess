import { ExclamationTriangleIcon } from '@heroicons/react/24/solid';
import { Component, ErrorInfo, PropsWithChildren, ReactNode } from 'react';

export function ErrorScreen({ error }: Readonly<{ error: unknown }>) {
	return (
		<div className="h-full w-full flex flex-col justify-center items-center gap-6">
			<ExclamationTriangleIcon className="text-white h-16 w-16" />
			<p className="text-white font-black text-center max-w-screen-lg">{`${error}`}</p>
		</div>
	);
}

export type ErrorBoundaryProps = Readonly<
	PropsWithChildren<{
		fallback: (error: unknown) => ReactNode;
	}>
>;

export class ErrorBoundary extends Component<ErrorBoundaryProps, { error: unknown }> {
	constructor(props: ErrorBoundaryProps) {
		super(props);
		this.state = { error: undefined };
	}

	componentDidCatch(error: Error, errorInfo: ErrorInfo) {
		console.error(error, errorInfo);
		this.setState({ error });
	}

	render() {
		if (this.state.error !== undefined) {
			return this.props.fallback(this.state.error);
		}

		return this.props.children;
	}
}
