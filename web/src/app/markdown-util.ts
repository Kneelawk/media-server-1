import { MarkedOptions, MarkedRenderer } from "ngx-markdown";
import { AnchorService } from "./anchor.service";

export function markedOptionsFactory(anchorService: AnchorService): MarkedOptions {
  const renderer = new MarkedRenderer();

  renderer.link = (href: string, title: string, text: string) => {
    return MarkedRenderer.prototype.link.call(renderer, anchorService.normalizeExternalUrl(href), title, text);
  }

  return {
    renderer: renderer
  };
}
