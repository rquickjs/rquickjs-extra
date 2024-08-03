export {};

declare global {
  /**
   * The URLSearchParams interface defines utility methods to work with the query string of a URL.
   */
  class URLSearchParams implements Iterable<[string, string]> {
    constructor(
      init?:
        | URLSearchParams
        | string
        | { readonly [name: string]: string }
        | Iterable<readonly [name: string, value: string]>
        | ReadonlyArray<readonly [name: string, value: string]>,
    );
    /**
     * Append a new name-value pair to the query string.
     */
    append(name: string, value: string): void;
    /**
     * If `value` is provided, removes all name-value pairs
     * where name is `name` and value is `value`.
     *
     * If `value` is not provided, removes all name-value pairs whose name is `name`.
     */
    delete(name: string, value?: string): void;
    /**
     * Returns an ES6 `Iterator` over each of the name-value pairs in the query.
     * Each item of the iterator is a JavaScript `Array`. The first item of the `Array` is the `name`, the second item of the `Array` is the `value`.
     *
     * Alias for `urlSearchParams[@@iterator]()`.
     */
    entries(): IterableIterator<[string, string]>;
    /**
     * Iterates over each name-value pair in the query and invokes the given function.
     *
     * ```js
     * const myURL = new URL('https://example.org/?a=b&#x26;c=d');
     * myURL.searchParams.forEach((value, name) => {
     *   console.log(name, value);
     * });
     * // Prints:
     * //   a b
     * //   c d
     * ```
     * @param fn Invoked for each name-value pair in the query
     */
    forEach(fn: (value: string, name: string) => void): void;
    /**
     * Returns the value of the first name-value pair whose name is `name`. If there
     * are no such pairs, `null` is returned.
     * @return or `null` if there is no name-value pair with the given `name`.
     */
    get(name: string): string | null;
    /**
     * Returns the values of all name-value pairs whose name is `name`. If there are
     * no such pairs, an empty array is returned.
     */
    getAll(name: string): string[];
    /**
     * Checks if the `URLSearchParams` object contains key-value pair(s) based on `name` and an optional `value` argument.
     *
     * If `value` is provided, returns `true` when name-value pair with
     * same `name` and `value` exists.
     *
     * If `value` is not provided, returns `true` if there is at least one name-value
     * pair whose name is `name`.
     */
    has(name: string, value?: string): boolean;
    /**
     * Returns an ES6 `Iterator` over the names of each name-value pair.
     *
     * ```js
     * const params = new URLSearchParams('foo=bar&#x26;foo=baz');
     * for (const name of params.keys()) {
     *   console.log(name);
     * }
     * // Prints:
     * //   foo
     * //   foo
     * ```
     */
    keys(): IterableIterator<string>;
    /**
     * Sets the value in the `URLSearchParams` object associated with `name` to `value`. If there are any pre-existing name-value pairs whose names are `name`,
     * set the first such pair's value to `value` and remove all others. If not,
     * append the name-value pair to the query string.
     *
     * ```js
     * const params = new URLSearchParams();
     * params.append('foo', 'bar');
     * params.append('foo', 'baz');
     * params.append('abc', 'def');
     * console.log(params.toString());
     * // Prints foo=bar&#x26;foo=baz&#x26;abc=def
     *
     * params.set('foo', 'def');
     * params.set('xyz', 'opq');
     * console.log(params.toString());
     * // Prints foo=def&#x26;abc=def&#x26;xyz=opq
     * ```
     */
    set(name: string, value: string): void;
    /**
     * The total number of parameter entries.
     */
    readonly size: number;
    /**
     * Sort all existing name-value pairs in-place by their names. Sorting is done
     * with a [stable sorting algorithm](https://en.wikipedia.org/wiki/Sorting_algorithm#Stability), so relative order between name-value pairs
     * with the same name is preserved.
     *
     * This method can be used, in particular, to increase cache hits.
     *
     * ```js
     * const params = new URLSearchParams('query[]=abc&#x26;type=search&#x26;query[]=123');
     * params.sort();
     * console.log(params.toString());
     * // Prints query%5B%5D=abc&#x26;query%5B%5D=123&#x26;type=search
     * ```
     */
    sort(): void;
    /**
     * Returns the search parameters serialized as a string, with characters
     * percent-encoded where necessary.
     */
    toString(): string;
    /**
     * Returns an ES6 `Iterator` over the values of each name-value pair.
     */
    values(): IterableIterator<string>;
    [Symbol.iterator](): IterableIterator<[string, string]>;
  }
}
