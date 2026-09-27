package {
    import flash.display.MovieClip;
    import flash.geom.Matrix;

    public class Test extends MovieClip {
        private var props = ["rotation", "x", "y", "scaleX", "scaleY"];

        public function Test() {
            TestZeroSkew();

            TestWeirdMatrix();
        }

        public function TestZeroSkew() {
            var limits1 = [0, NaN];
            var limits2 = [1, NaN];
            var limits3 = [1, 0];
            var rots = [0, 0.5, -0.5, 1, -1, 2, -2, 0.25, -0.25];
            for each (var prop in ["rotation"]) {
                trace("// TestZeroSkew: " + prop + " = 0");
                for(var idx1 = 0; idx1 < 2; idx1++) {
                    for(var idx2 = 0; idx2 < 2; idx2++) {
                        for(var idxr = 0; idxr < rots.length; idxr++) {
                            var m = new Matrix();

                            var clip = new MovieClip();
                            m.identity();
                            m.scale(limits1[idx1], limits1[idx2]);
                            m.rotate(rots[idxr] * Math.PI);
                            clip.transform.matrix = m;
                            trace(m);
                            printChange(clip, prop, 0);
                            trace("");

                            m.identity();
                            m.scale(limits2[idx1], limits2[idx2]);
                            m.rotate(rots[idxr] * Math.PI);
                            clip.transform.matrix = m;
                            trace(m);
                            printChange(clip, prop, 0);
                            trace("");

                            m.identity();
                            m.scale(limits3[idx1], limits3[idx2]);
                            m.rotate(rots[idxr] * Math.PI);
                            clip.transform.matrix = m;
                            trace(m);
                            printChange(clip, prop, 0);
                            trace("");
                        }
                    }
                }
            }
        }

        public function TestWeirdMatrix() {
            var limits1 = [0, NaN];
            var limits2 = [1, NaN];
            var limits3 = [1, 0];
            var limits4 = [-1, 0];
            for each (var prop in props) {
                trace("// " + prop + " = 0");
                for(var idx1 = 0; idx1 < 2; idx1++) {
                    for(var idx2 = 0; idx2 < 2; idx2++) {
                        for(var idx3 = 0; idx3 < 2; idx3++) {
                            for(var idx4 = 0; idx4 < 2; idx4++) {
                                var clip = new MovieClip();
                                clip.transform.matrix = new Matrix(limits1[idx1], limits1[idx2], limits1[idx3], limits1[idx4], 0, 0);
                                printChange(clip, prop, 0);
                                trace("");
                                clip.transform.matrix = new Matrix(limits2[idx1], limits2[idx2], limits2[idx3], limits2[idx4], 0, 0);
                                printChange(clip, prop, 0);
                                trace("");
                                clip.transform.matrix = new Matrix(limits3[idx1], limits3[idx2], limits3[idx3], limits3[idx4], 0, 0);
                                printChange(clip, prop, 0);
                                trace("");
                                clip.transform.matrix = new Matrix(limits4[idx1], limits4[idx2], limits4[idx3], limits4[idx4], 0, 0);
                                printChange(clip, prop, 0);
                                trace("");
                            }
                        }
                    }
                }
            }
            return;

            var limits = [0, NaN, Infinity, -Infinity];
            for each (var prop in props) {
                trace("// " + prop + " = NaN");
                for each (var l1 in limits) {
                    for each (var l2 in limits) {
                        for each (var l3 in limits) {
                            for each (var l4 in limits) {
                                var clip = new MovieClip();
                                clip.transform.matrix = new Matrix(l1, l2, l3, l4, 0, 0);
                                printChange(clip, prop, 0);
                                trace("");
                            }
                        }
                    }
                }
            }
            return;

            for each (var prop in props) {
                trace("// " + prop + " = Infinity");
                var clip = new MovieClip();
                clip.transform.matrix = new Matrix(2, 0, 4, 0, 5, 6);
                printChange(clip, prop, Infinity);

                clip = new MovieClip();
                var newMat = new Matrix(2, 1, 4, 1, 5, 6);
                clip.transform.matrix = newMat;
                printChange(clip, prop, Infinity);

                clip = new MovieClip();
                var newMat = new Matrix(7, 0, 9, 0, 11, 12);
                clip.transform.matrix = newMat;
                printChange(clip, prop, Infinity);

                trace("");
                trace("");
            }
        }

        private function skewMatrix(skew_x: Number, skew_y: Number):Matrix {
            return new Matrix(0, Math.tan(skew_y), Math.tan(skew_x), 1)
        }

        private function printChange(clip:MovieClip, prop:String, value:*) {
            trace("  transform.matrix = " + clip.transform.matrix);
            var result = "";
            for each (var p in props) {
                result += p + "=" + clip[p] + ", ";
            }
            trace("  " + result);

            trace("clip[" + prop + "] = " + value);
            clip[prop] = value;

            result = "";
            for each (var p in props) {
                result += p + "=" + clip[p] + ", ";
            }
            trace("  " + result);
            trace("  transform.matrix = " + clip.transform.matrix);
        }
    }
}
