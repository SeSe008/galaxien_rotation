"use strict";

/**
 * Renders formula with KaTeX in element.
 *
 * @param {string} formula
 * @param {HTMLElement} element
 */
export function render_katex(formula, element) {
    console.log(`Rendering ${formula}`);
    katex.render(formula, element, { throwOnError: false });
}
