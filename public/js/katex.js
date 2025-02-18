"use strict";

/**
 * Renders equation with KaTeX in element.
 *
 * @param {string} equation
 * @param {HTMLElement} element
 */
export function render_katex(equation, element) {
    console.log(`Rendering ${equation}`);
    katex.render(equation, element, { throwOnError: false });
}
