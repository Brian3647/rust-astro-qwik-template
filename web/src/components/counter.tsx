import { useSignal, component$ } from '@builder.io/qwik';

export const Counter = component$(() => {
	const counter = useSignal(0);

	return (
		<>
			Counter:{' '}
			<button onClick$={() => counter.value++}>{counter.value}</button>
		</>
	);
});
