export function require<T>(value: T): NonNullable<T> {
	return value!;
}
