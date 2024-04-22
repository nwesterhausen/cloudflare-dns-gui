import { type Component, createEffect, createSignal, type JSX } from "solid-js";
import { useUserProvider } from "./providers/UserContext";
import { useTokenProvider } from "./providers/TokenContext";

export const Toaster: Component = () => {
	const [toast, setToast] = createSignal<JSX.Element>(<div />);
	const putToast = (token: JSX.Element) => {
		setToast(token);
		setTimeout(() => {
			setToast(<div />);
		}, 5_000);
	};

	const [apiToken] = useTokenProvider();
	const userCtx = useUserProvider();
	if (!userCtx) return <div />;
	const [details] = userCtx;

	createEffect(() => {
		if (apiToken().length > 0 && details.latest.email && details.latest.email.length > 0) {
			putToast(
				<div class="alert alert-success">
					<div class="flex flex-col gap-3">
						<div>API Key is valid.</div>
						<div>User: {details.latest.email}</div>
						<div>Organizations: {details.latest.organizations.join(", ")}</div>
					</div>
				</div>,
			);
		}
	});

	return <div class="toast toast-top toast-end">{toast()}</div>;
};
