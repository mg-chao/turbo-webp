<div>

  <h1><code>turbo-webp</code></h1>

<strong>Ultra-fast WebP encoder and decoder, based on Rust's image-webp implementation</strong>

### Web Worker Demo

##### getPixels.ts

```ts
let wasmModuleArrayBuffer: ArrayBuffer;

export async function getPixels(
    imageBuffer: ArrayBuffer,
    width: number,
    height: number,
): Promise<ImageData> {
    return new Promise(async (resolve, reject) => {
        const worker = new Worker(new URL('./getPixelsWorker.ts', import.meta.url));

        worker.onmessage = async (event) => {
            resolve(event.data);
            worker.terminate();
        };

        worker.onerror = (error) => {
            reject(error);
            worker.terminate();
        };

        if (!wasmModuleArrayBuffer) {
            const wasmModuleResponse = await fetch(
                new URL('turbo-webp/turbo_webp_bg.wasm', import.meta.url),
            );
            wasmModuleArrayBuffer = await wasmModuleResponse.arrayBuffer();
        }
        worker.postMessage({ imageBuffer, width, height, wasmModuleArrayBuffer });
    });
}

```

##### getPixelsWorker.ts

```ts
import { decode, initSync } from 'turbo-webp';

self.onmessage = async (
    event: MessageEvent<{
        wasmModuleArrayBuffer: ArrayBuffer;
        imageBuffer: ArrayBuffer;
        width: number;
        height: number;
    }>,
) => {
    const { imageBuffer, width, height, wasmModuleArrayBuffer } = event.data;

    initSync({
        module: wasmModuleArrayBuffer,
    });

    const imageData = decode(new Uint8Array(imageBuffer));

    self.postMessage(new ImageData(new Uint8ClampedArray(imageData), width, height));
};
```